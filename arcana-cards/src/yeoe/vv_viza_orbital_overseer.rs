//! Vv'viza, Orbital Overseer — `{1}{G}{W}{U}` 4/4 Legendary Jellyfish Advisor.
//! Flying, Vigilance.
//! When Vv'viza enters, conjure a card of your choice from the Planets
//! Spellbook onto the battlefield. (Conjure not modeled — GAP'd.)
//! Whenever you attack, create a Lander token. It becomes a 2/1 artifact
//! creature and gains flying.

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
    let name = reg.interner_mut().intern("Vv'viza, Orbital Overseer");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let advisor = reg.interner_mut().intern("Advisor");
    let _lander = reg.interner_mut().intern("Lander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "conjure a card from the Planets Spellbook" — Conjure not
            // modeled; effect empty.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_conjure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "Whenever you attack" has no dedicated trigger; SelfAttacks
            // is the closest. The Lander is minted directly as the 2/1 flying
            // artifact creature it becomes.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_lander,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_conjure(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled.
    Vec::new()
}

fn attack_make_lander(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let lander = reg.interner().lookup("Lander").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: lander,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
