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


WS.sendRequest(findTestObject('aValidate OTP', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid]))



response1 = WS.sendRequest(findTestObject('aValidate Mobile Number'))


//response1 = WS.sendRequestAndVerify(findTestObject('aValidate OTP', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid]))
//response1 = WS.sendRequest(findTestObject('aValidate OTP'))
def slurper1 = new groovy.json.JsonSlurper()

//def result1 = slurper1.parseText(response1.getResponseText())
def result1 = slurper1.parseText(response1.responseBodyContent)

println('***************************************')

println('THIS IS THE RESPONSE FROM VALIDATEOTP TEST CASE = ' + result1)

println('***************************************')

def value1 = result1.userId

GlobalVariable.userid = value1

println('***************************************')

println('THE USER ID IS GENARATED  = ' + GlobalVariable.userid)

println('***************************************')

