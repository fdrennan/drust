# drust

```
autossh -f -nNT -i /home/fdrennan/ndexr.pem -R 8787:localhost:8787 ubuntu@ec2-18-218-56-75.us-east-2.compute.amazonaws.com -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -o ExitOnForwardFailure=yesb
```

ssh -i "ndexr.pem" 