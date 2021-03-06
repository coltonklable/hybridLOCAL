<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PNRBFManagement_Create PNR</name>
   <tag></tag>
   <elementGuidId>2bed295e-c84e-414c-a4b6-625da8de39fc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic R1dTX1VBVF9QNzExMzE2MDpOZXdwYXNzXzR5b3U=</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitXmlOnSession>
         &lt;web:Token>${TokenNumber}&lt;/web:Token>
         &lt;web:Request>
            &lt;PNRBFManagement_49>
               &lt;InsertSegAfterMods>
                  &lt;SegNum>00&lt;/SegNum>
               &lt;/InsertSegAfterMods>
               &lt;AirSegSellMods>
                  &lt;AirSegSell>
                     &lt;Vnd>${Carrier}&lt;/Vnd>
                     &lt;FltNum>${FlightNumber}&lt;/FltNum>
                     &lt;OpSuf/>
                     &lt;Class>Y&lt;/Class>
                     &lt;StartDt>${StartDate}&lt;/StartDt>
                     &lt;StartAirp>${Origin}&lt;/StartAirp>
                     &lt;EndAirp>${Destination}&lt;/EndAirp>
                     &lt;Status>NN&lt;/Status>
                     &lt;NumPsgrs>01&lt;/NumPsgrs>
                     &lt;StartTm>${StartTime}&lt;/StartTm>
                     &lt;EndTm>${EndTime}&lt;/EndTm>
                     &lt;DtChg>00&lt;/DtChg>
                     &lt;StopoverIgnoreInd/>
                     &lt;AvailDispType>G&lt;/AvailDispType>
                     &lt;VSpec/>
                     &lt;AvailJrnyNum>01&lt;/AvailJrnyNum>
                     &lt;SponsoredFltLineNum/>
                     &lt;NeutralFltLineNum>00&lt;/NeutralFltLineNum>
                     &lt;SponsoredFltDBID/>
                  &lt;/AirSegSell>
                 &lt;/AirSegSellMods>
               &lt;PNRBFPrimaryBldChgMods>
                  &lt;ItemAry>
                     &lt;Item>
                        &lt;DataBlkInd>N&lt;/DataBlkInd>
                        &lt;NameQual>
                           &lt;EditTypeInd>A&lt;/EditTypeInd>
                           &lt;EditTypeIndAppliesTo/>
                           &lt;AddChgNameRmkQual>
                              &lt;NameType/>
                              &lt;LNameID>01&lt;/LNameID>
                              &lt;LName>SINATRA&lt;/LName>
                              &lt;LNameRmk/>
                              &lt;NameTypeQual>
                                 &lt;FNameAry>
                                    &lt;FNameItem>
                                       &lt;PsgrNum>01&lt;/PsgrNum>
                                       &lt;AbsNameNum>01&lt;/AbsNameNum>
                                       &lt;FName>FRANK&lt;/FName>
                                       &lt;FNameRmk/>
                                    &lt;/FNameItem>
                                 &lt;/FNameAry>
                              &lt;/NameTypeQual>
                           &lt;/AddChgNameRmkQual>
                        &lt;/NameQual>
                     &lt;/Item>
                     &lt;Item>
                        &lt;DataBlkInd>P&lt;/DataBlkInd>
                        &lt;PhoneQual>
                           &lt;EditTypeInd>A&lt;/EditTypeInd>
                           &lt;AddPhoneQual>
                              &lt;City>DEN&lt;/City>
                              &lt;Type>B&lt;/Type>
                              &lt;PhoneNumber>303-555-1212&lt;/PhoneNumber>
                           &lt;/AddPhoneQual>
                        &lt;/PhoneQual>
                     &lt;/Item>
                     &lt;Item>
                        &lt;DataBlkInd>W&lt;/DataBlkInd>
                        &lt;AddrQual>
                           &lt;EditTypeInd>A&lt;/EditTypeInd>
                           &lt;AddChgAddr>
                              &lt;Addr>FRANK SINATRA @ 123 PEARL ST @ DENVER CO Z/80209&lt;/Addr>
                           &lt;/AddChgAddr>
                        &lt;/AddrQual>
                     &lt;/Item>
                     &lt;Item>
                        &lt;DataBlkInd>T&lt;/DataBlkInd>
                        &lt;TkQual>
                           &lt;Tk>${TicketDate}&lt;/Tk>
                           &lt;!--The appropriate format for Galileo would be: TAU/01AUG-->
                        &lt;/TkQual>
                     &lt;/Item>
                     &lt;Item>
                        &lt;DataBlkInd>E&lt;/DataBlkInd>
                        &lt;EndMarkQual>
                           &lt;EndMark>E&lt;/EndMark>
                        &lt;/EndMarkQual>
                     &lt;/Item>
                  &lt;/ItemAry>
               &lt;/PNRBFPrimaryBldChgMods>
               &lt;PNRBFSecondaryBldChgMods>
                  &lt;ItemAry>
                     &lt;Item>
                        &lt;DataBlkInd>S&lt;/DataBlkInd>
                        &lt;SSRQual>
                           &lt;EditTypeInd>A&lt;/EditTypeInd>
                           &lt;AddQual>
                              &lt;SSRCode>DOCS&lt;/SSRCode>
                              &lt;LNameNum>01&lt;/LNameNum>
                              &lt;PsgrNum>01&lt;/PsgrNum>
                              &lt;AbsNameNum>01&lt;/AbsNameNum>
                              &lt;FltNum/>
                              &lt;OpSuf/>
                              &lt;AirV>${Carrier}&lt;/AirV>
                              &lt;Dt>${StartDate}&lt;/Dt>
                              &lt;BIC>Y&lt;/BIC>
                              &lt;StartAirp>${Origin}&lt;/StartAirp>
                              &lt;EndAirp>${Destination}&lt;/EndAirp>
                              &lt;Text>P/DL/S12345678/US/12JUL66/M/23OCT15/SINATRA/FRANK&lt;/Text>
                           &lt;/AddQual>
                        &lt;/SSRQual>
                     &lt;/Item>
                     &lt;Item>
                        &lt;DataBlkInd>F&lt;/DataBlkInd>
                        &lt;FOPQual>
                           &lt;EditTypeInd>A&lt;/EditTypeInd>
                           &lt;AddChgQual>
                              &lt;TypeInd>1&lt;/TypeInd>
                              &lt;VarLenQual>
                                 &lt;FOP>S&lt;/FOP>
                              &lt;/VarLenQual>
                           &lt;/AddChgQual>
                        &lt;/FOPQual>
                     &lt;/Item>
                     &lt;Item>
                        &lt;DataBlkInd>E&lt;/DataBlkInd>
                        &lt;EndQual>
                           &lt;EndMark>E&lt;/EndMark>
                        &lt;/EndQual>
                     &lt;/Item>
                  &lt;/ItemAry>
               &lt;/PNRBFSecondaryBldChgMods>
               &lt;EndTransactionMods>
                  &lt;EndTransactRequest>
                     &lt;ETInd>E&lt;/ETInd>
                     &lt;RcvdFrom>PASSENGER&lt;/RcvdFrom>
                  &lt;/EndTransactRequest>
               &lt;/EndTransactionMods>
            &lt;/PNRBFManagement_49>
         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXmlOnSession>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Username</defaultValue>
      <description></description>
      <id>1805e0b6-3074-4ff2-bf1c-209a352a830a</id>
      <masked>false</masked>
      <name>Username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PCC</defaultValue>
      <description></description>
      <id>0cc317ec-eff4-49e8-9490-bbc5e5968047</id>
      <masked>false</masked>
      <name>PCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Provider</defaultValue>
      <description></description>
      <id>039a26ed-7d5e-4bbc-8aa5-066d60d35db2</id>
      <masked>false</masked>
      <name>Provider</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>8384aa5b-5ca2-4cf5-ab9d-c581492e73cc</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FlightNumber1</defaultValue>
      <description></description>
      <id>306cdd05-be21-4991-a55a-743ec00e3ed5</id>
      <masked>false</masked>
      <name>FlightNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Origin</defaultValue>
      <description></description>
      <id>ef98554a-4288-4b00-8f9f-aa05393b3d43</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Destination</defaultValue>
      <description></description>
      <id>c2ff2772-6402-47d5-82bb-16fc074160d4</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartDate1</defaultValue>
      <description></description>
      <id>c2f440ab-c332-452a-8b3b-8383946a3d38</id>
      <masked>false</masked>
      <name>StartDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.StartTime1</defaultValue>
      <description></description>
      <id>6adc41bd-a27f-496c-923a-73c7daae6a1d</id>
      <masked>false</masked>
      <name>StartTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndTime1</defaultValue>
      <description></description>
      <id>3e65ce01-192c-48b7-a459-a258739e191f</id>
      <masked>false</masked>
      <name>EndTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TokenNumber</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>TokenNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TicketDate</defaultValue>
      <description></description>
      <id>d0c60f85-df16-48a6-af5b-2baa8914329d</id>
      <masked>false</masked>
      <name>TicketDate</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
