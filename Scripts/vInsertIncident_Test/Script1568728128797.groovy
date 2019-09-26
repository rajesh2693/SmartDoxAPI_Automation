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

def incidenTID = UUID.randomUUID().toString().toUpperCase().replace('-', '')

println(incidenTID)

def now = new Date()

incidenTDate = now.format('dd-MM-YYYY HH:mm:ss +5:30')

println(incidenTDate)

 WS.sendRequest(findTestObject('uInsert Incident', [('userId') : GlobalVariable.userid, ('uniqueToken') : GlobalVariable.uniquetoken
            , ('incidentID') : incidenTID, ('incidentDate') : incidenTDate, ('checkInLocality') : checkInLocality, ('incidentLocality') : incidentLocality]))

responsev = WS.sendRequestAndVerify(findTestObject('uInsert Incident', [('userId') : GlobalVariable.userid, ('uniqueToken') : GlobalVariable.uniquetoken
            , ('incidentID') : incidenTID, ('incidentDate') : incidenTDate, ('checkInLocality') : checkInLocality, ('incidentLocality') : incidentLocality]))

def slurperv = new groovy.json.JsonSlurper()

def resultv = slurperv.parseText(responsev.getResponseText())

println('***************************************')

println('THIS IS THE RESPONSE FROM ISERT INCIDENT TEST CASE = ' + resultv)

println('***************************************')

