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

def now = new Date()

fromDatE = now.format('dd-MM-YYYY HH:mm:ss +5:30')

println(fromDatE)

toDatE = now.format('dd-MM-YYYY HH:mm:ss +5:30')

println(toDatE)

WS.sendRequest(findTestObject('x1Target Break Down', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid
            , ('fromDatE') : fromDatE, ('toDatE') : toDatE]))

responsex1 = WS.sendRequestAndVerify(findTestObject('x1Target Break Down', [('uniqueToken') : GlobalVariable.uniquetoken
            , ('userId') : GlobalVariable.userid, ('fromDatE') : fromDatE, ('toDatE') : toDatE]))


def slurperx1 = new groovy.json.JsonSlurper()

def resultx1 = slurperx1.parseText(responsex1.getResponseText())

println('***************************************')

println('THIS IS THE RESPONSE FROM TARGET BREAKDOWN TEST CASE = ' + resultx1)

println('***************************************')

