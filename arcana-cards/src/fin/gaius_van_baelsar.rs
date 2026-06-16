//! Gaius van Baelsar — `{2}{B}{B}` Legendary 3/2 black Human Soldier.
//!
//! Rules text:
//! * When Gaius van Baelsar enters, choose one —
//!     • Each player sacrifices a creature token of their choice.
//!     • Each player sacrifices a nontoken creature of their choice.
//!     • Each player sacrifices an enchantment of their choice.
//!
//! This is a MODAL TRIGGERED ability ("choose one" on an ETB trigger). The
//! demonstrated modal machinery (`ModalSpec` / `dispatch_modal_effect` /
//! `with_mode_effects`) is only wired for SPELL abilities — `TriggeredAbilityDef`
//! has no modal field, so the per-mode choice can't be posted from a trigger
//! with the demonstrated API. The trigger itself is emitted; its effect is
//! GAP'd because the engine offers no way to make the modal selection from a
//! triggered ability here (and emitting any single mode unconditionally would
//! be wrong).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Gaius van Baelsar");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_modal_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_modal_sacrifice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" on a triggered ability — TriggeredAbilityDef has
    //       no modal field, so the mode selection (each player sacrifices a
    //       creature token / nontoken creature / enchantment of their choice)
    //       cannot be posted with the demonstrated API.
    Vec::new()
}
