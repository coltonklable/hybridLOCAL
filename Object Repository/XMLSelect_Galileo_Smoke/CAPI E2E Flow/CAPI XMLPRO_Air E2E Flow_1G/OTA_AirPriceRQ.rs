<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OTA_AirPriceRQ</name>
   <tag></tag>
   <elementGuidId>d634b102-5f68-4398-9e3c-2029719527de</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;SOAP-ENV:Header>
      &lt;t:Transaction xmlns:t=&quot;xxs&quot;>
         &lt;tc>
            &lt;iden u=&quot;${Username}&quot; p=&quot;${Password}&quot;/>
            &lt;provider session=&quot;${Session}&quot; pcc=&quot;${PCC}&quot;>Worldspan&lt;/provider>
            &lt;trace/>
         &lt;/tc>
      &lt;/t:Transaction>
   &lt;/SOAP-ENV:Header>
   &lt;SOAP-ENV:Body>
      &lt;ns1:ProviderTransaction xmlns:ns1=&quot;xxs&quot;>
         &lt;REQ>
            &lt;OTA_AirPriceRQ Type=&quot;UserPrice&quot; Version=&quot;1&quot; xmlns=&quot;http://www.opentravel.org/OTA/2003/05&quot;>
               &lt;POS>
                  &lt;Source ISOCountry=&quot;US&quot;/>
               &lt;/POS>
               &lt;BookingReferenceID Type=&quot;14&quot; ID=&quot;&quot;/>
               &lt;AirItinerary DirectionInd=&quot;Oneway&quot;>
                  &lt;OriginDestinationOptions>
                     &lt;OriginDestinationOption RefNumber=&quot;1&quot;>
                        &lt;FlightSegment FlightNumber=&quot;${FlightNumber}&quot; DepartureDateTime=&quot;${DepartureDateTime}&quot;>
                           &lt;DepartureAirport LocationCode=&quot;${Origin}&quot;/>
                           &lt;ArrivalAirport LocationCode=&quot;${Destination}&quot;/>
                           &lt;MarketingAirline Code=&quot;${Carrier}&quot;/>
                        &lt;/FlightSegment>
                     &lt;/OriginDestinationOption>
                  &lt;/OriginDestinationOptions>
               &lt;/AirItinerary>
               &lt;TravelerInfoSummary>
                  &lt;PriceRequestInformation CurrencyCode=&quot;USD&quot; TicketingCountry=&quot;US&quot; SaleCountry=&quot;US&quot;/>
               &lt;/TravelerInfoSummary>
&lt;!--               &lt;FlightReference FlightRefNumber=&quot;1&quot;/>
-->
            &lt;/OTA_AirPriceRQ>
         &lt;/REQ>
      &lt;/ns1:ProviderTransaction>
   &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Session</defaultValue>
      <description></description>
      <id>a5cfd42a-6953-4424-b734-d8f851e56190</id>
      <masked>false</masked>
      <name>Session</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PCC</defaultValue>
      <description></description>
      <id>ac6baefa-85e0-482f-b2c7-2f07b49356f7</id>
      <masked>false</masked>
      <name>PCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Password</defaultValue>
      <description></description>
      <id>414e9fe0-4feb-4ab4-b80b-02d2cfbdab2b</id>
      <masked>false</masked>
      <name>Password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Username</defaultValue>
      <description></description>
      <id>7e952581-3f2f-4821-b03a-cb84edbd4936</id>
      <masked>false</masked>
      <name>Username</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(2, 1)</defaultValue>
      <description></description>
      <id>156d3e38-cf05-4e77-820c-bc9ed7620ee3</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(3, 1)</defaultValue>
      <description></description>
      <id>bca40c1c-f925-433a-893c-fbdb3f17e4dc</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepartureDateTime</defaultValue>
      <description></description>
      <id>97f55060-6c59-4ba9-ae2e-0891fb827e41</id>
      <masked>false</masked>
      <name>DepartureDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>f8238d97-b6a4-4123-b1d6-b8f525e71667</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FlightNumber1</defaultValue>
      <description></description>
      <id>267aac9b-d213-4307-a37a-5eec8e523e7f</id>
      <masked>false</masked>
      <name>FlightNumber</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
