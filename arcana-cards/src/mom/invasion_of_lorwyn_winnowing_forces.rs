//! Invasion of Lorwyn // Winnowing Forces
//!
//! Front face: {4}{B}{G} Battle — Siege with 6 defense counters.
//! ETB: Destroy target non-Elf creature an opponent controls with power X or
//! less, where X is the number of lands you control.
//!
//! Back face (Winnowing Forces): Creature — Elf Warrior.
//! Power and toughness are each equal to the number of lands you control.
//!
//! Defeat→back-face is auto-wired by the engine SBA. Front ETB (the targeted
//! destroy) is face-gated to the battle face (0).
//!
//! Notes:
//! - "power X or less, where X = lands you control" power cap is applied at
//!   resolution via script::power_of + script::count_matching.
//! - Back-face P/T "each equal to the number of lands you control" is a Layer 7a
//!   self-CDA (`ContinuousEffect::self_pt_from_match` over lands you control),
//!   installed on ETB and gated to the back face via
//!   `Duration::WhileSourceShowsFace(1)` (dormant on the battle face, live once
//!   the defeat transform flips it); the back bones carry PtValue::Star.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Lorwyn");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Winnowing Forces — Creature — Elf Warrior
    let back_name = reg.interner_mut().intern("Winnowing Forces");
    let elf_sub = reg.interner_mut().intern("Elf");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elf_sub);
    back_subtypes.0.insert(warrior_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        // P/T each equal to the number of lands you control (Layer 7a self-CDA,
        // installed on ETB, gated to the back face); Star marks the bones.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // ETB targets an opponent non-Elf creature ("power X or less" gate is
    // applied at resolution via power check).
    let etb_target = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature().without_subtype_sym(elf_sub),
        ),
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_resolve,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_target],
            })
            // Back-face CDA: P/T each equal to the number of lands you control
            // (live only while showing the back face).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_back_land_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front targeted-destroy fires only on the battle face; the CDA
            // installer (id 2) runs on ETB so the back-face P/T is ready when
            // the defeat transform flips the face.
            .with_trigger_face_gate(1, 0)
            .with_transform_back(back_face),
    )
}

/// Layer 7a self-CDA gated to the back face (Winnowing Forces): P/T each equal
/// to the number of lands you control.
fn install_back_land_cda(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceShowsFace(1),
        ),
    }]
}

fn etb_resolve(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // X = number of lands you control
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let x = script::count_matching(state, &land_filter, trig.controller);
    // Gate: target's power must be ≤ X
    let target_power = script::power_of(state, *id);
    if target_power > x as i32 {
        return Vec::new();
    }
    vec![Effect::DestroyPermanent { target: *id }]
}
