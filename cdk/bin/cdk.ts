#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { CdkStack } from "../lib/cdk-stack";

const app = new cdk.App();

let stackName = `${process.env.PROJECT}-${process.env.SERVICE}-${process.env.ENV}-stack`;
if (process.env.STACK) {
  stackName = process.env.STACK;
}

new CdkStack(app, stackName, {
  stackName,

  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
});
