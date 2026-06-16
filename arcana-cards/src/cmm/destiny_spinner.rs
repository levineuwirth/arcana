//! Destiny Spinner — `{1}{G}` 2/3 Enchantment Creature — Human.
//! Creature and enchantment spells you control can't be countered (static —
//! GAP).
//! `{3}{G}: Target land you control becomes an X/X Elemental creature with
//! trample and haste until end of turn, where X is the number of enchantments
//! you control. It's still a land.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Destiny Spinner");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "Creature and enchantment spells you control can't be
    // countered" — uncounterable-grant has no documented Effect/replacement.

    let your_land = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}: Target land you control becomes an X/X Elemental creature with trample \
                   and haste until end of turn, where X is the number of enchantments you \
                   control. It's still a land."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(your_land),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: animate_land,
        }),
    )
}

fn animate_land(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ENCHANTMENT.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) as i32;
    // GAP: the "Elemental" creature subtype grant is not expressible (no
    // add-subtype Effect). Type/base-P/T/keywords are applied below; it stays
    // a land since AddType is additive.
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: *id,
            power: n,
            toughness: n,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
