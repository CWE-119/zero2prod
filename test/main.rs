#[cfg(test)]
mod test{
	use crate::health_check;

	#[tokio::test]
	async fn health_check_succeeds(){
		let response = health_check().await;

		assert!(response.status().is_success)
	}
}