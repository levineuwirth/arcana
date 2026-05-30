//! Invasion of Lorwyn // Winnowing Forces
//!
//! Front face: {4}{B}{G} Battle — Siege with 6 defense counters.
//! ETB: Destroy target non-Elf creature an opponent controls with power X or
//! less, where X is the number of lands you control.
//!
//! Back face (Winnowing Forces): Creature — Elf Warrior.
//! Power and toughness are each equal to the number of lands you control.
//!
//! GAPs:
//! - "non-Elf" restriction on the target is not enforceable in TargetFilter
//!   (no subtype-exclusion filter); we target any opponent creature and apply
//!   the power check at resolution. The non-Elf restriction is a fidelity gap.
//! - "power X or less, where X = lands you control" power cap is also applied
//!   at resolution via script::power_of + script::count_matching.
//! - Back face "P/T = number of lands you control" — dynamic static layer not
//!   expressible; back face is registered without P/T (default).
//! - GAP: defeat→cast-back-face not auto-wired (CR 310.11).

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
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
        // GAP: "P/T = number of lands you control" — dynamic P/T not wired.
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // ETB targets an opponent creature (non-Elf restriction is a GAP at
    // targeting time; applied partially at resolution via power check).
    let etb_target = TargetRequirement {
        filter: TargetFilter::Creature,
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
            .with_transform_back(back_face),
    )
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
    // GAP: non-Elf check not enforced at targeting or resolution (no subtype
    // exclusion available via script API for opponent's creatures).
    vec![Effect::DestroyPermanent { target: *id }]
}
