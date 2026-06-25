//! Invasion of New Capenna // Holy Frazzle-Cannon — `{W}{B}` Battle — Siege
//! with 3 defense counters.
//!
//! Front face: When this Siege enters, you may sacrifice an artifact or
//! creature. When you do, exile target artifact or creature an opponent controls.
//!
//! Back face (Holy Frazzle-Cannon): Legendary Artifact — Equipment.
//! Whenever equipped creature attacks, put a +1/+1 counter on that creature
//! and each other creature you control that shares a creature type with it.
//! Equip {1}.
//!
//! # GAPs
//! - Siege ETB: "you may sacrifice an artifact or creature. When you do,
//!   exile target artifact or creature an opponent controls." — the
//!   OptionalPaymentKind API only supports Mana and Life costs; a
//!   sacrifice-conditional exile is not expressible. Emitting a simplified
//!   ETB that exiles a target artifact or creature an opponent controls
//!   (without the sacrifice cost gate). GAP: sacrifice cost gate not modeled.
//! - Back face "whenever equipped creature attacks, put +1/+1 counter on it
//!   and each other creature you control that shares a creature type with it"
//!   — equipped-creature-attacks trigger with a dynamic "shares a creature type
//!   with the equipped creature" board sweep is not expressible (no host-relative
//!   shared-subtype filter); back-face-only triggered ability not modeled (GAP).
//! - Back face Equip {1}: wired via `with_equip_face_gated({1}, 1)` (face-gated
//!   to the back/Equipment face; the Equip keyword is recorded on that face).
//!
//! Defeat→back-face is auto-wired by the engine SBA. Front ETB (the simplified
//! exile) is face-gated to the battle face (0). The back-face equipped-creature-
//! attacks trigger remains GAP'd (host-relative shared-subtype board sweep).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of New Capenna");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Holy Frazzle-Cannon — Legendary Artifact — Equipment
    let back_name = reg.interner_mut().intern("Holy Frazzle-Cannon");
    let equipment_sub = reg.interner_mut().intern("Equipment");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(equipment_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // ETB trigger: exile target artifact or creature an opponent controls.
    // GAP: full oracle requires "you may sacrifice an artifact or creature.
    // When you do, exile ..." — sacrifice cost gate not modelable with
    // OptionalPaymentKind. Simplified to unconditional exile on ETB.
    let opponent_perm_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::Opponent);
    let etb_req = TargetRequirement {
        filter: TargetFilter::Permanent(opponent_perm_filter),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 3,
            })
            .with_transform_back(back)
            // Back face "Holy Frazzle-Cannon": Equip {1} (face-gated to the
            // back/Equipment face).
            .with_equip_face_gated(ManaCost::parse("{1}").expect("valid equip cost"), 1)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_req],
            })
            .with_trigger_face_gate(1, 0),
        // GAP: back-face-only triggered ability (equipped creature attacks →
        //      +1/+1 counter on it and each shared-creature-type creature) not
        //      modeled — host-relative shared-subtype board sweep not expressible
    )
}

fn etb_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}
