//! Champion of Stray Souls — `{4}{B}{B}` 4/4 Skeleton Warrior.
//!
//! * `{3}{B}{B}, {T}, Sacrifice X other creatures: Return X target creature
//!   cards from your graveyard to the battlefield.` — the variable-X
//!   sacrifice cost ("Sacrifice X other creatures") is not an expressible
//!   activation cost (no variable sacrifice count + matching variable target
//!   count linkage), so the whole ability is GAP'd.
//! * `{5}{B}{B}: Put this card from your graveyard on top of your library.` —
//!   a graveyard-activated ability that puts this card on top of the library.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Champion of Stray Souls");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "{3}{B}{B}, {T}, Sacrifice X other creatures: Return X target creature
    // cards from your graveyard to the battlefield." — variable-X sacrifice cost
    // tied to a variable-X target count is not expressible with the activation
    // cost / target shapes available.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{B}{B}: Put this card from your graveyard on top of your library.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: put_on_top,
            }),
    )
}

fn put_on_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutOnTopOfLibrary { target: ctx.source }]
}
