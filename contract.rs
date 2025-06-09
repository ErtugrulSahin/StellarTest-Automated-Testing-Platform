use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct TestPlatform;

#[contracttype]
pub struct TestResult {
    pub tester: Address,
    pub contract: Symbol,
    pub passed: bool,
    pub details: Symbol,
}

#[contractimpl]
impl TestPlatform {
    fn results<'a>(env: &'a Env) -> Vec<'a, TestResult> {
        env.storage().instance().get::<Vec<TestResult>>(Symbol::short("results")).unwrap_or(Vec::new(&env))
    }

    pub fn submit_result(env: Env, contract: Symbol, passed: bool, details: Symbol) {
        let tester = env.invoker();
        let mut results = Self::results(&env);
        results.push_back(TestResult { tester, contract, passed, details });
        env.storage().instance().set(Symbol::short("results"), &results);
    }

    pub fn get_results(env: Env) -> Vec<TestResult> {
        Self::results(&env)
    }
}
