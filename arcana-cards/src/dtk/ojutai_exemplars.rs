//! Ojutai Exemplars — `{2}{W}{W}` 4/4 Creature — Human Monk.
//!
//! Whenever you cast a noncreature spell, choose one —
//! • Tap target creature.
//! • This creature gains first strike and lifelink until end of turn.
//! • Exile this creature, then return it to the battlefield tapped under its
//!   owner's control.
//!
//! The "choose one" modal selection is only wired for SPELL abilities
//! (`ModalSpec` / `dispatch_modal_effect` on `with_spell_ability`); a
//! TriggeredAbilityDef has no modal field. The modal CHOICE is therefore
//! GAP'd. As a documented partial we emit the self-only mode (gain first
//! strike and lifelink until end of turn) — the two modes needing a chosen
//! target / exile-return path are not expressible from a non-modal trigger.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Ojutai Exemplars");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(arcana_core::targets::ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_noncreature_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_noncreature_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" not expressible from a triggered ability (modal
    // dispatch is spell-only). Emitting the self-only mode as a partial.
    vec![Effect::Pump {
        target: trig.source,
        power: 0,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
    }]
}
