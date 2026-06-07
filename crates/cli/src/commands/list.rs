use std::iter;

use common::{
    domain::container::Container, port::container::ContainerRuntime,
    service::container::ContainerService,
};

pub async fn ContainerList() -> Vec<Container> {
    todo!()
}

pub async fn findContainer<R>(service: &ContainerService<R>, locator: &str) -> Result<Container, ()>
where
    R: ContainerRuntime + Send + Sync,
{
    let containers = service.list().await;

    match containers {
        Ok(n) => {
            let options: Vec<String> = n.into_iter().map(|k| k.name()).collect();
            let result = fuzzy::seach_list(locator, options.iter().map(|f| f.as_str()).collect());

            println!("{result:?}");
        }
        Err(_) => return Err(()),
    }

    // fuzzy::seach_list(locator, options)
    todo!()
}
