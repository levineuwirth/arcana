//! Voldaren Bloodcaster // Bloodbat Summoner — `{1}{B}` Vampire Wizard 2/1 with Flying.
//! Front: Flying.
//!   Whenever this creature or another nontoken creature you control dies, create a Blood token.
//!   GAP: "Whenever you create a Blood token, if you control five or more Blood tokens, transform
//!   this creature." — no TokenCreated trigger condition is available.
//! Back (Bloodbat Summoner): Flying 4/3 Vampire Wizard.
//!   GAP: back-face-only triggered ability (combat trigger) not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voldaren Bloodcaster");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);
    subtypes.0.insert(wizard_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Bloodbat Summoner");
    let back_vampire = reg.interner_mut().intern("Vampire");
    let back_wizard = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_vampire);
    back_subtypes.0.insert(back_wizard);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Blood token name for resolver use
    let _blood_name = reg.interner_mut().intern("Blood");

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: whenever a nontoken creature you control dies, create a Blood token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: create_blood_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_blood_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Blood token: colorless artifact named "Blood".
    // GAP: Blood token's activated ability ("{1}, {T}, Discard a card, Sacrifice this token:
    // Draw a card.") not wired — discard-cost activation not expressible.
    let blood_name = match reg.interner().lookup("Blood") {
        Some(n) => n,
        None => return Vec::new(),
    };
    let token = TokenDefinition {
        name: blood_name,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: SubtypeSet::default(),
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
