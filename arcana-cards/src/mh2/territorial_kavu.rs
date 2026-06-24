//! Territorial Kavu — `{R}{G}` */* Kavu.
//! "Domain — Territorial Kavu's power and toughness are each equal to
//! the number of basic land types among lands you control." — GAP. `*` is the
//! number of DISTINCT basic land types (Plains/Island/Swamp/Mountain/Forest)
//! among your lands. `self_pt_from_match` counts matching permanents (not
//! distinct subtypes) and `self_pt_cda`'s compute has no registry to resolve
//! the basic-type subtype symbols, so neither self-CDA constructor can express
//! a distinct-basic-land-type count. P/T left as `*`.
//! "Whenever this creature attacks, choose one — …" — a MODAL
//! triggered ability; modal dispatch is only available for spell
//! abilities in this card class, so the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Territorial Kavu");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    // GAP: keyword "Domain" — not a modeled KeywordAbility; the
    // power/toughness CDA it defines is left as `*`.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "choose one — • Discard a card. If you do, draw a card. •
    // Exile up to one target card from a graveyard." — modal choice on a
    // TRIGGERED ability is not expressible (modal dispatch is spell-only).
    Vec::new()
}
