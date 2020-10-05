resource "aws_dynamodb_table" "table" {
  name         = "scones-ie"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "TransactionID"

  attribute {
    name = "UserId"
    type = "S"
  }
}
