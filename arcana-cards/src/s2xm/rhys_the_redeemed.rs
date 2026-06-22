//! Rhys the Redeemed — `{G/W}` 1/1 Legendary Elf Warrior.
//! {2}{G/W}, {T}: Create a 1/1 green and white Elf Warrior creature token.
//! {4}{G/W}{G/W}, {T}: For each creature token you control, create a token
//! that's a copy of that creature.
//!
//! Two activated abilities: a mana+tap token maker, and a mana+tap
//! "copy each creature token" using `CopyPermanent` per matching id wrapped
//! in a `Sequence` (per the catalog note: `ForEach` is a count primitive and
//! doesn't substitute per id for copies).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rhys the Redeemed");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G/W}, {T}: Create a 1/1 green and white Elf Warrior creature token."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G/W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_elf_warrior,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G/W}{G/W}, {T}: For each creature token you control, create a token that's a copy of that creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G/W}{G/W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_creature_tokens,
            }),
    )
}

fn make_elf_warrior(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elf = reg.interner().lookup("Elf").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: elf,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn copy_creature_tokens(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        ctx.controller,
    );
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::Sequence(
        ids.into_iter()
            .map(|id| Effect::CopyPermanent { target: id })
            .collect(),
    )]
}
