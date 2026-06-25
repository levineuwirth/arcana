//! Toralf, God of Fury // Toralf's Hammer
//!
//! Front face: `{2}{R}{R}` Legendary Creature — God, 5/4.
//!   Trample.
//!   Whenever a creature or planeswalker an opponent controls is dealt excess noncombat
//!   damage, Toralf deals damage equal to the excess to any target other than that permanent.
//!
//! Back face: `{1}{R}` Legendary Artifact — Equipment.
//!   Equipped creature has "{1}{R}, {T}, Unattach Toralf's Hammer: It deals 3 damage to any
//!   target. Return Toralf's Hammer to its owner's hand."
//!   Equipped creature gets +3/+0 as long as it's legendary.
//!   Equip {1}{R}
//!
//! GAP: Front trigger "excess noncombat damage" — no accessor for excess damage in script;
//!      trigger registered but effect is empty (dynamic amount not computable).
//! GAP: "deals damage equal to the excess" — DamageDealt trigger doesn't expose excess amount.
//! GAP: Back-face granted ability "Equipped creature has '{1}{R},{T}, Unattach Toralf's
//!      Hammer: It deals 3 damage to any target. Return Toralf's Hammer to its owner's
//!      hand.'" — "Unattach (the source equipment)" is not an expressible ActivationCost,
//!      so the granted activated ability is omitted. The Equip ability (via
//!      `with_equip_face_gated({1}{R}, 1)`) and the +3/+0 legendary buff ARE wired below.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toralf, God of Fury");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // Back face: Toralf's Hammer — Legendary Artifact — Equipment
    let back_name = reg.interner_mut().intern("Toralf's Hammer");
    let equipment_sub = reg.interner_mut().intern("Equipment");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(equipment_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
            colors: ColorSet::red(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            // MDFC back: `with_equip_face_gated` only records the Equip keyword onto
            // a Transform back face, so record it here on the MDFC back manually.
            keywords: vec![KeywordAbility::Equip(ManaCost::parse("{1}{R}").expect("valid cost"))],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Back face "Toralf's Hammer": Equip {1}{R} (face-gated to the back/Equipment
            // face; MDFC back-cast sets visible_face=1 so face 1 gates correctly).
            .with_equip_face_gated(ManaCost::parse("{1}{R}").expect("valid equip cost"), 1)
            // Triggered: whenever a creature or planeswalker an opponent controls is dealt
            // excess noncombat damage, Toralf deals damage equal to the excess to any target.
            // GAP: excess noncombat damage amount not accessible via script; effect is empty.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    combat_only: false,
                },
                intervening_if: None,
                effect: excess_damage_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_trigger_face_gate(1, 0)
            // Back face (Toralf's Hammer): on entering as the Equipment, install
            // "equipped creature gets +3/+0 as long as it's legendary" — a dynamic
            // attached pump that follows the equipment's attachment and re-reads the
            // host's legendary status each layer pass. Inert while unattached / on the
            // front creature face (attached_to is None there).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_legendary_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1),
        // GAP: granted activated ability "Equipped creature has '{1}{R},{T}, Unattach
        //      Toralf's Hammer: deal 3 damage to any target. Return Toralf's Hammer to
        //      its owner's hand.'" — "Unattach (the source equipment)" is not an
        //      expressible ActivationCost; the granted ability is omitted.
    )
}

/// Install the "+3/+0 as long as the equipped creature is legendary" dynamic pump,
/// anchored to the Equipment so it follows re-equips and expires when it leaves play.
fn install_legendary_buff(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            legendary_buff,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// +3/+0 when the equipped (attached) creature is legendary, else +0/+0.
fn legendary_buff(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(host) = state.object_or_lki(source).and_then(|o| o.attached_to) else {
        return (0, 0);
    };
    let Some(host_obj) = state.object_or_lki(host) else {
        return (0, 0);
    };
    if host_obj.characteristics.supertypes.0 & SupertypeSet::LEGENDARY != 0 {
        (3, 0)
    } else {
        (0, 0)
    }
}

fn excess_damage_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "damage equal to the excess" — excess noncombat damage amount not accessible
    // via any script:: helper; cannot compute dynamic damage. Effect omitted.
    Vec::new()
}
