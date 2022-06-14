<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PNRBFManagement_StorePrice</name>
   <tag></tag>
   <elementGuidId>b97b7670-ce8a-4fb2-826a-b8b5e46ac83e</elementGuidId>
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
            &lt;PNRBFManagement_53>
               &lt;StorePriceMods>
                  &lt;PlatingAirVMod>
                     &lt;AirV>${Carrier}&lt;/AirV>
                  &lt;/PlatingAirVMod>
                  &lt;PassengerType>
                     &lt;PsgrAry>
                        &lt;Psgr>
                           &lt;LNameNum>01&lt;/LNameNum>
                           &lt;PsgrNum>01&lt;/PsgrNum>
                           &lt;AbsNameNum>01&lt;/AbsNameNum>
                           &lt;PTC>ADT&lt;/PTC>
                           &lt;PricePTCOnly>Y&lt;/PricePTCOnly>
                           &lt;TIC/>
                        &lt;/Psgr>
                     &lt;/PsgrAry>
                  &lt;/PassengerType>
                  &lt;AssocPsgrs>
                     &lt;PsgrAry>
                        &lt;Psgr>
                           &lt;LNameNum>01&lt;/LNameNum>
                           &lt;PsgrNum>01&lt;/PsgrNum>
                           &lt;AbsNameNum>01&lt;/AbsNameNum>
                        &lt;/Psgr>
                     &lt;/PsgrAry>
                  &lt;/AssocPsgrs>
                  &lt;SegSelection>
                     &lt;ReqAirVPFs>Y&lt;/ReqAirVPFs>
                     &lt;SegRangeAry>
                        &lt;SegRange>
                           &lt;StartSeg>00&lt;/StartSeg>
                           &lt;EndSeg>00&lt;/EndSeg>
                           &lt;FareType>P&lt;/FareType>
                           &lt;PFQual>
                              &lt;CRSInd>${Provider}&lt;/CRSInd>
                              &lt;PCC>${PCC}&lt;/PCC>
                              &lt;AirV/>
                              &lt;Contract/>
                              &lt;PublishedFaresInd>Y&lt;/PublishedFaresInd>
                              &lt;Type>A&lt;/Type>
                           &lt;/PFQual>
                        &lt;/SegRange>
                     &lt;/SegRangeAry>
                  &lt;/SegSelection>
               &lt;/StorePriceMods>
               &lt;EndTransactionMods>
                  &lt;EndTransactRequest>
                     &lt;ETInd>E&lt;/ETInd>
                     &lt;RcvdFrom>PASSENGER&lt;/RcvdFrom>
                  &lt;/EndTransactRequest>
               &lt;/EndTransactionMods>
            &lt;/PNRBFManagement_53>
         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXmlOnSession>
      &lt;web:BeginSession/>
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
      <defaultValue>GlobalVariable.TokenNumber</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>TokenNumber</name>
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
