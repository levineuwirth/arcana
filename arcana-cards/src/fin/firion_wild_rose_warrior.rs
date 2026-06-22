//! Firion, Wild Rose Warrior — `{2}{R}` 3/3 Legendary Human Rebel Warrior (red).
//! Equipped creatures you control have haste.
//! Whenever a nontoken Equipment you control enters, create a token that's a
//! copy of it, except it has "This Equipment's equip abilities cost {2} less to
//! activate." Sacrifice that token at the beginning of the next upkeep.
//!
//! The first line is a pure STATIC continuous ability — GAP'd. The trigger
//! mints a token copy of the entering Equipment via Effect::CopyPermanent. The
//! equip-cost-reduction rider and the "sacrifice that token at the next upkeep"
//! rider cannot be attached to a CopyPermanent token (the minted token's id is
//! engine-internal and unavailable to schedule against) — both GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Firion, Wild Rose Warrior");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(warrior);

    let equipment_filter = script::subtype_filter(reg, "Equipment")
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Equipped creatures you control have haste." (continuous ability).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: equipment_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: copy_equipment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_equipment(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    // GAP: the copy's "equip abilities cost {2} less" rider and the
    // "sacrifice that token at the next upkeep" rider are not expressible on a
    // CopyPermanent token (no rider field; minted token id is engine-internal).
    vec![Effect::CopyPermanent { target: id }]
}
