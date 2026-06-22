//! Emrakul's Messenger — `{1}{U}` 2/1 colorless Eldrazi Faerie Rogue (Devoid).
//! Flying.
//! Whenever you draw your second card each turn, create a 0/1 colorless Eldrazi
//! Spawn creature token with "Sacrifice this token: Add {C}."
//!
//! Devoid makes the card colorless (colors: ColorSet::colorless()). Flying is a
//! base keyword. The "second card each turn" trigger is approximated with
//! CardDrawn{You} + OncePerTurn. The token's granted "Sacrifice: Add {C}"
//! ability cannot be expressed on a TokenDefinition (GAP).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emrakul's Messenger");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let _spawn = reg.interner_mut().intern("Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger — "your SECOND card each turn" has no dedicated
            // variant; CardDrawn{You} + OncePerTurn approximates (fires on the
            // first draw of the turn, not specifically the second).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_spawn(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Spawn").expect("Spawn interned");
    let mut subtypes = SubtypeSet::default();
    let eldrazi = reg.interner().lookup("Eldrazi").expect("Eldrazi interned");
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(spawn);
    // GAP: the granted "Sacrifice this token: Add {C}" ability cannot be
    // expressed on a TokenDefinition; emit the bare 0/1 token.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spawn,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
