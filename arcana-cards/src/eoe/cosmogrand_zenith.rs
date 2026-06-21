//! Cosmogrand Zenith — `{2}{W}` 2/4 Human Soldier.
//!
//! Oracle:
//! * Whenever you cast your second spell each turn, choose one —
//!   • Create two 1/1 white Human Soldier creature tokens.
//!   • Put a +1/+1 counter on each creature you control.
//!
//! "Second spell each turn" is modeled with `SpellCast { caster: You }`
//! gated by a CR 603.4 intervening-if that fires only when this is the
//! second spell cast this turn (`spells_cast_this_turn == 2`).
//!
//! GAP: the modal "choose one" machinery (dispatch_modal_effect /
//! with_mode_effects) is only available for SPELL abilities, not for a
//! triggered ability. This trigger is modeled as the first mode (create
//! two 1/1 white Human Soldier tokens); the mode choice and the second
//! mode (+1/+1 counter on each creature you control) are omitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosmogrand Zenith");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: Some(if_second_spell),
            effect: make_two_soldiers,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "your second spell each turn" — fires only when exactly two spells
/// have been cast this turn (this cast being the second).
fn if_second_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let all = arcana_core::targets::ObjectFilter::new();
    script::spells_cast_this_turn(s, &all, you) == 2
}

/// Mode 1: create two 1/1 white Human Soldier creature tokens.
fn make_two_soldiers(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").expect("Human interned");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
    // GAP: mode choice + mode 2 (+1/+1 counter on each creature you control)
    // not expressible on a triggered ability.
}
