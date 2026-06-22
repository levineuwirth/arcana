//! Halsin, Emerald Archdruid — `{3}{G}` 2/4 Legendary Creature — Elf Druid.
//! "{1}: Until end of turn, target token you control becomes a green Bear
//!  creature with base power and toughness 4/4 in addition to its other
//!  colors and types.
//!  Choose a Background"
//!
//! "Choose a Background" is not a supported KeywordAbility variant (the
//! second-commander mechanic is not modeled), so it's GAP'd. The
//! activated ability targets a token you control and (until end of turn)
//! sets base P/T 4/4, adds the creature type, and sets it green. Two
//! fidelity gaps: `SetColor` replaces the color set rather than adding
//! green to existing colors, and there is no add-subtype effect so the
//! "Bear" subtype is not granted. The base-P/T and become-a-creature parts
//! are faithful.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Halsin, Emerald Archdruid");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    // GAP: keyword — "Choose a Background" (second-commander mechanic) is
    // not a supported KeywordAbility variant.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}: Until end of turn, target token you control becomes a green Bear creature with base power and toughness 4/4 in addition to its other colors and types."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .tokens_only()
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
            effect: become_bear,
        }),
    )
}

fn become_bear(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        // GAP: no add-subtype effect — the "Bear" subtype is not granted.
        Effect::SetColor {
            target: *id,
            colors: ColorSet::green(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: *id,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
    ])]
}
