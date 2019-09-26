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

import com.kms.katalon.core.logging.KeywordLogger


def dte = new Date()

fromDatE = dte.format('dd-MM-YYYY HH:mm:ss +5:30')

println(fromDatE)


bte = new Date()

toDatE = bte.format('dd-MM-YYYY HH:mm:ss +5:30')

println(toDatE)

WS.sendRequest(findTestObject('ySupervisior Team Status', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid
            , ('fromDatE') : fromDatE, ('toDatE') : toDatE]))

responsez1 = WS.sendRequestAndVerify(findTestObject('ySupervisior Team Status', [('uniqueToken') : GlobalVariable.uniquetoken
            , ('userId') : GlobalVariable.userid, ('fromDatE') : fromDatE, ('toDatE') : toDatE]))

def slurperz1 = new groovy.json.JsonSlurper()

def resultz1 = slurperz1.parseText(responsez1.getResponseText())

println('***************************************')

println('THIS IS THE RESPONSE FROM SUPERVISIOR TEAM STATUS TEST CASE = ' + resultz1)

println('***************************************')



