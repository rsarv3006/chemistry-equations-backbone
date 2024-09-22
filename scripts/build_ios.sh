#!/bin/bash


rm -rf ChemistryEquationsBackbone

rm -rf ../BackboneSPM/Sources
rm -rf ../BackboneSPM/RustFramework.xcframework

cargo swift package


cp -R ChemistryEquationsBackbone/Sources ../BackboneSPM/Sources
cp -R ChemistryEquationsBackbone/RustFramework.xcframework ../BackboneSPM/RustFramework.xcframework
cp ChemistryEquationsBackbone/Package.swift ../BackboneSPM/Package.swift
