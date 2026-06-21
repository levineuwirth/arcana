//! Sage of the Maze — `{2}{G}` 1/3 Creature — Elf Wizard.
//!
//! Oracle:
//! * {T}: Add two mana in any combination of colors. (GAP: no
//!   player-chosen-combination AddMana primitive.)
//! * {T}: Until end of turn, target land you control becomes an X/X Citizen
//!   creature with haste in addition to its other types, where X is twice the
//!   number of Gates you control. Activate only as a sorcery.
//! * Tap an untapped Gate you control: Untap this creature.
//!
//! Decomposition: three `ActivatedAbilityDef`s. Ability 1's any-color mana is
//! GAP'd. Ability 2 animates the target land (SetBasePT to X/X, add the
//! creature type, grant Haste); the Citizen *subtype* add has no effect
//! primitive and is a fidelity GAP. Ability 3 uses a tap-other Gate cost to
//! untap this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage of the Maze");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add two mana in any combination of colors.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Until end of turn, target land you control becomes an X/X \
                       Citizen creature with haste in addition to its other types, where \
                       X is twice the number of Gates you control. Activate only as a \
                       sorcery."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped Gate you control: Untap this creature.".into(),
                cost: ActivationCost {
                    tap_other: Some(
                        script::subtype_filter(reg, "Gate")
                            .controlled_by(ControllerConstraint::You),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_self,
            }),
    )
}

fn add_any_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Add two mana in any combination of colors" — no
    // player-chosen-combination AddMana primitive.
    Vec::new()
}

fn animate_land(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // X = twice the number of Gates you control.
    let gates = script::count_matching(
        state,
        &script::subtype_filter(reg, "Gate").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let x = (gates * 2) as i32;
    // GAP fidelity: the Citizen *subtype* is not added (no add-subtype effect);
    // the X/X creature body, creature type, and haste are modeled.
    vec![Effect::Sequence(vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: *id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ])]
}

fn untap_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
