//! Ghalma the Shaper — `{1}{W}{W}` 3/3 legendary Elephant Artificer
//! with Vigilance. "Whenever Ghalma the Shaper attacks, conjure a
//! card named Tempered Steel into your hand, then create a 1/1
//! colorless Myr artifact creature token."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghalma the Shaper");
    let elephant = reg.interner_mut().intern("Elephant");
    let artificer = reg.interner_mut().intern("Artificer");
    let _myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "conjure a card named Tempered Steel into your hand" — Conjure
    //      is an Arena-only mechanic with no Effect variant.
    let myr = reg.interner().lookup("Myr").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: myr,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
