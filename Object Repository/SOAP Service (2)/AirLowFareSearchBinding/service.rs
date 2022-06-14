<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>e53f6ce4-6a6b-4dba-9996-cbf420690e04</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>useHASpath</name>
      <type>Main</type>
      <value>HCA</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ses=&quot;http://www.travelport.com/soa/common/security/SessionContext_v1&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
   &lt;soapenv:Body>
      &lt;air:LowFareSearchReq TargetBranch=&quot;P7113929&quot; SolutionResult=&quot;true&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:CityOrAirport Code=&quot;DEN&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:CityOrAirport Code=&quot;ORD&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime PreferredTime=&quot;2021-12-07&quot;/>
         &lt;/air:SearchAirLeg>
       &lt;!--  &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:CityOrAirport Code=&quot;CHC&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:CityOrAirport Code=&quot;AKL&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime PreferredTime=&quot;2021-11-25&quot;/>
         &lt;/air:SearchAirLeg>
-->
         &lt;air:AirSearchModifiers>
            &lt;air:PreferredProviders>
               &lt;com:Provider Code=&quot;1G&quot;/>
            &lt;/air:PreferredProviders>
           &lt;!-- &lt;air:PermittedCarriers>
               &lt;com:Carrier Code=&quot;KL&quot;/>
            &lt;/air:PermittedCarriers>
-->
         &lt;/air:AirSearchModifiers>
         &lt;com:SearchPassenger Code=&quot;ADT&quot;/>
         &lt;!--&lt;com:SearchPassenger Code=&quot;CHD&quot;/>-->
      &lt;/air:LowFareSearchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/C:/Users/ramraja.sampathkumar/Desktop/SVT/POC_REST_SOAP/UAPI/wsdl/air_v49_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
