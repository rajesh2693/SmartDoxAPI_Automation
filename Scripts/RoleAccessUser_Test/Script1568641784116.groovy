import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable



WS.sendRequestAndVerify(findTestObject('Role Access By User', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid]))

responseR = WS.sendRequestAndVerify(findTestObject('Role Access By User'))


def slurperR = new groovy.json.JsonSlurper()

def resultR = slurperR.parseText(responseR.getResponseText())

println('***************************************')

println('THIS IS THE RESPONSE FROM ROLE ACESS USER  TEST CASE = ' + resultR)

println('***************************************')

def valueR = resultR.roleId

GlobalVariable.loGID = valueR

println('***************************************')

println('THE USER LOG ID IS   = ' + GlobalVariable.loGID)

println('***************************************')