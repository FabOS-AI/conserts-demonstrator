#!/bin/sh

# SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
#
# SPDX-License-Identifier: MIT


conserts compile --aas-json --ros1 -i ./scanner/scanner.yml
cp ./target/consert_scanner/Consert.json ./target/scanner_ConsertSubmodel.json

conserts compile --aas-json --ros1 --provider ./scanner/scanner.yml -i ./workspace/workspace.yml
cp ./target/consert_workspace/Consert.json ./target/workspace_ConsertSubmodel.json

conserts compile --aas-json --ros1 --provider ./scanner/scanner.yml --provider ./workspace/workspace.yml -i ./robot/robot.yml
cp ./target/consert_robot/Consert.json ./target/robot_ConsertSubmodel.json

conserts compile --aas-json --ros1 --provider ./scanner/scanner.yml --provider ./workspace/workspace.yml --provider ./robot/robot.yml -i ./binpicking/binpicking.yml
cp ./target/consert_binpicking/Consert.json ./target/binpicking_ConsertSubmodel.json
