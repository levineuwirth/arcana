//! Fanatic of Rhonas — `{1}{G}` 1/4 Snake Druid (green).
//! {T}: Add {G}.
//! Ferocious — {T}: Add {G}{G}{G}{G}. Activate only if you control a creature
//! with power 4 or greater.
//! Eternalize {2}{G}{G}.
//!
//! Neither "Ferocious" nor "Eternalize" is a KeywordAbility variant, so the
//! keyword line is empty. The two mana abilities are wired. The Ferocious
//! "activate only if you control a creature with power 4+" precondition has no
//! constructor in the available surface — GAP'd (the ability is otherwise
//! identical). Eternalize (graveyard activation minting a modified token copy)
//! is not expressible — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fanatic of Rhonas");
    let snake = reg.interner_mut().intern("Snake");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: Eternalize {2}{G}{G} — graveyard activation minting a modified token copy; not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_one_green,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Activate only if you control a creature with power 4 or greater" precondition has no constructor in the available surface.
                text: "Ferocious — {T}: Add {G}{G}{G}{G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_four_green,
            }),
    )
}

fn add_one_green(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn add_four_green(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); 4],
    }]
}
