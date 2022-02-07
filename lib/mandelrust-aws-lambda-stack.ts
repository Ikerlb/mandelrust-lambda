import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import { RetentionDays } from '@aws-cdk/aws-logs';
import * as apigw from '@aws-cdk/aws-apigateway';

export class MandelrustAwsLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props)

    // Function that calls Rust
    const mandelrust = new lambda.Function(this, 'mandelrust', {
      description:
        'Deploying a Rust function on Lambda using the custom runtime',
      code: lambda.Code.fromAsset(
        'resources/target/x86_64-unknown-linux-musl/release/lambda'
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'not.required',
      environment: {
	    GIT_VERSION: "3badf1b7",
        RUST_BACKTRACE: '1',
      },
      logRetention: RetentionDays.ONE_WEEK,
    })

	new apigw.LambdaRestApi(this, 'mandelrustEndpoint', {
		handler: mandelrust,
	});
  }
}
