//! Voice of Resurgence — `{G}{W}` 2/2 green/white Creature — Elemental.
//! "Whenever an opponent casts a spell during your turn and when this
//! creature dies, create a green and white Elemental creature token with
//! 'This token's power and toughness are each equal to the number of
//! creatures you control.'"
//!
//! GAP (trigger 1): SpellCast condition has no "during your turn" constraint;
//! the trigger will fire on any opponent spell cast, not just during your turn.
//! GAP (token static ability): The token's dynamic P/T ("equal to the number
//! of creatures you control") is not expressible in TokenDefinition — the token
//! is created as a fixed 1/1 placeholder.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Voice of Resurgence");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: no "during your turn" constraint on SpellCast; fires on all
                // opponent spells regardless of whose turn it is.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_elemental_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg
        .interner()
        .lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    // GAP: The token's printed static ability "this token's power and toughness
    // are each equal to the number of creatures you control" is not expressible
    // in TokenDefinition. Emitted as a fixed 1/1 placeholder.
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
