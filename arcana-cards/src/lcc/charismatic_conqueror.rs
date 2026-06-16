//! Charismatic Conqueror — `{1}{W}` 2/2 Vampire Soldier with Vigilance.
//! "Whenever an artifact or creature an opponent controls enters
//! untapped, they may tap that permanent. If they don't, you create a
//! 1/1 white Vampire creature token with lifelink."
//!
//! The "they may tap that permanent, otherwise you create a token"
//! opponent-choice gate has no demonstrated wrapper, so the token is
//! created unconditionally (the relevant payoff side).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charismatic Conqueror");
    let vampire = reg.interner_mut().intern("Vampire");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::Opponent)
        .untapped_only();

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: make_vampire,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

// GAP: "they may tap that permanent. If they don't, …" — opponent
// optional-tap gate is not expressible; the token is created
// unconditionally.
fn make_vampire(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: vampire,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Lifelink],
            abilities: vec![],
        },
    }]
}
