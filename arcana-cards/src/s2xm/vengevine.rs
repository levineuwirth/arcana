//! Vengevine — `{2}{G}{G}` 4/3 Elemental with Haste.
//! "Whenever you cast a spell, if it's the second creature spell you cast this
//! turn, you may return this card from your graveyard to the battlefield."
//!
//! Decomposition:
//! * Haste — keyword.
//! * Graveyard recursion — a `SpellCast` (you) trigger that fires from the
//!   graveyard, gated by an intervening-if (the just-cast spell is the second
//!   creature spell you cast this turn), returning this card from the graveyard
//!   to the battlefield. ("may" is a resolution-time choice; minor fidelity
//!   gap.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengevine");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: Some(if_second_creature_spell),
            effect: return_self_from_graveyard,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_second_creature_spell(
    state: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // The just-cast spell is already counted, so "the second creature spell you
    // cast this turn" means the running count is exactly 2.
    let creature_filter = ObjectFilter::creature();
    script::spells_cast_this_turn(state, &creature_filter, you) == 2
}

fn return_self_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
