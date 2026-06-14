use crate::error::{error, log_redefinition_error};
use crate::syntax_tree::expression::{
    Assignment, AssignmentTarget, BinaryExpr, Call, Expr, Get, GroupingExpr, LogicalExpr, Set,
    UnaryExpr, Variable,
};
use crate::syntax_tree::statement::{
    Block, ClassDecl, FunctionDecl, If, Print, Return, Stmt, Var, While,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
}

pub struct Resolver {
    error_encountered: bool,
    current_fn_type: FunctionType,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            error_encountered: false,
            current_fn_type: FunctionType::None,
            scopes: vec![HashMap::new()],
        }
    }

    pub fn has_encountered_errors(&self) -> bool {
        self.error_encountered
    }

    pub fn resolve_statements(&mut self, stmts: &mut [Stmt]) {
        for stmt in stmts {
            self.resolve_statement(stmt);
        }
    }
}

impl Resolver {
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_variable(&mut self, name: &str, line: usize) {
        match self.scopes.last_mut() {
            None => {}
            Some(scope) => {
                if scope.contains_key(name) {
                    log_redefinition_error(name, line);
                    self.error_encountered = true;
                }
                scope.insert(name.to_string(), false);
            }
        }
    }

    fn define_variable(&mut self, name: &str) {
        match self.scopes.last_mut() {
            None => {}
            Some(scope) => {
                scope.insert(name.to_string(), true);
            }
        }
    }
}

// Resolve statements
impl Resolver {
    fn resolve_statement(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Block(block) => self.resolve_block_stmt(block),
            Stmt::Class(class) => self.resolve_class_stmt(class),
            Stmt::Expr(expr) => self.resolve_expression(expr),
            Stmt::Function(func) => self.resolve_function_stmt(func),
            Stmt::If(if_) => self.resolve_if_stmt(if_),
            Stmt::Print(print) => self.resolve_print_stmt(print),
            Stmt::Return(return_) => self.resolve_return_stmt(return_),
            Stmt::While(while_) => self.resolve_while_stmt(while_),
            Stmt::Var(var) => self.resolve_var_stmt(var),
        }
    }

    fn resolve_block_stmt(&mut self, block: &mut Block) {
        self.begin_scope();
        for stmt in &mut block.stmts {
            self.resolve_statement(stmt);
        }
        self.end_scope();
    }

    fn resolve_class_stmt(&mut self, class: &mut ClassDecl) {
        self.declare_variable(&class.name, class.line);
        self.define_variable(&class.name);

        self.begin_scope();
        self.scopes.last_mut().unwrap().insert("this".to_string(), true);

        let mut encountered = HashSet::<String>::new();
        for method in &mut class.methods {
            if encountered.contains(&method.name) {
                log_redefinition_error(&method.name, method.line);
                self.error_encountered = true;
            }
            encountered.insert(method.name.clone());
            self.resolve_function(method, FunctionType::Method);
        }

        self.end_scope();
    }

    fn resolve_function_stmt(&mut self, func: &mut FunctionDecl) {
        self.declare_variable(&func.name, func.line);
        self.define_variable(&func.name);
        self.resolve_function(func, FunctionType::Function);
    }

    fn resolve_if_stmt(&mut self, if_stmt: &mut If) {
        self.resolve_expression(&mut if_stmt.condition);
        self.resolve_statement(&mut if_stmt.then_branch);
        if let Some(else_branch) = &mut if_stmt.else_branch {
            self.resolve_statement(else_branch);
        }
    }

    fn resolve_print_stmt(&mut self, print_stmt: &mut Print) {
        self.resolve_expression(&mut print_stmt.expr);
    }

    fn resolve_return_stmt(&mut self, return_stmt: &mut Return) {
        if self.current_fn_type == FunctionType::None {
            error(return_stmt.line, "Cannot return from top-level code");
        }

        if let Some(expr) = &mut return_stmt.expr {
            self.resolve_expression(expr);
        }
    }

    fn resolve_while_stmt(&mut self, while_stmt: &mut While) {
        self.resolve_expression(&mut while_stmt.condition);
        self.resolve_statement(&mut while_stmt.body);
    }

    fn resolve_var_stmt(&mut self, var: &mut Var) {
        self.declare_variable(&var.name, var.line);
        if let Some(init) = &mut var.initializer {
            self.resolve_expression(init);
        }
        self.define_variable(&var.name);
    }

    fn resolve_function(&mut self, func: &mut FunctionDecl, fn_type: FunctionType) {
        let cached_type = self.current_fn_type;
        self.current_fn_type = fn_type;

        self.begin_scope();
        for param in &func.params {
            self.declare_variable(&param.0, param.1);
            self.define_variable(&param.0);
        }
        for stmt in &mut func.body {
            self.resolve_statement(stmt);
        }
        self.end_scope();

        self.current_fn_type = cached_type;
    }
}

// Resolve expressions
impl Resolver {
    fn resolve_expression(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Literal(_) => {}
            Expr::Unary(unary) => self.resolve_unary_expr(unary),
            Expr::Binary(binary) => self.resolve_binary_expr(binary),
            Expr::Logical(logical) => self.resolve_logical_expr(logical),
            Expr::Call(call) => self.resolve_call_expr(call),
            Expr::Get(get) => self.resolve_get_expr(get),
            Expr::Set(set) => self.resolve_set_expr(set),
            Expr::This(this) => self.resolve_local_variable(this),
            Expr::Grouping(grouping) => self.resolve_grouping_expr(grouping),
            Expr::Variable(variable) => self.resolve_variable_expr(variable),
            Expr::Assignment(assignment) => self.resolve_assignment_expr(assignment),
        }
    }

    fn resolve_unary_expr(&mut self, unary_expr: &mut UnaryExpr) {
        self.resolve_expression(&mut unary_expr.expr);
    }

    fn resolve_binary_expr(&mut self, binary_expr: &mut BinaryExpr) {
        self.resolve_expression(&mut binary_expr.left);
        self.resolve_expression(&mut binary_expr.right);
    }

    fn resolve_logical_expr(&mut self, logical_expr: &mut LogicalExpr) {
        self.resolve_expression(&mut logical_expr.left);
        self.resolve_expression(&mut logical_expr.right);
    }

    fn resolve_call_expr(&mut self, call_expr: &mut Call) {
        self.resolve_expression(&mut call_expr.callee);
        for arg in &mut call_expr.args {
            self.resolve_expression(arg);
        }
    }

    fn resolve_get_expr(&mut self, get_expr: &mut Get) {
        self.resolve_expression(&mut get_expr.expr);
    }

    fn resolve_set_expr(&mut self, set_expr: &mut Set) {
        self.resolve_expression(&mut set_expr.value);
        self.resolve_expression(&mut set_expr.object);
    }

    fn resolve_grouping_expr(&mut self, grouping_expr: &mut GroupingExpr) {
        self.resolve_expression(&mut grouping_expr.expression);
    }

    fn resolve_variable_expr(&mut self, var: &mut Variable) {
        if !self.scopes.is_empty()
            && let Some(false) = self.scopes.last().unwrap().get(&var.name)
        {
            error(
                var.line,
                &format!(
                    "Cannot read local variable '{}' in it's own initializer",
                    var.name
                ),
            );
            self.error_encountered = true;
        }
        self.resolve_local_variable(var);
    }

    fn resolve_assignment_expr(&mut self, assignment: &mut Assignment) {
        self.resolve_expression(&mut assignment.value);
        match &mut assignment.target {
            AssignmentTarget::Variable(var) => self.resolve_local_variable(var),
        }
    }

    fn resolve_local_variable(&mut self, var: &mut Variable) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&var.name) {
                let distance = self.scopes.len() - 1 - i;
                var.local_distance = Some(distance);
                return;
            }
        }
    }
}
