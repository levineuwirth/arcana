//! Druid of the Sacred Beaker — `{2}{G}` 2/2 green Creature — Deer Bird Ape Druid.
//! {T}: Add {G} for each Crossbreed Labs watermark among permanents you control.
//! GAP: "Crossbreed Labs watermark" — watermark detection not in ObjectFilter or script API.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Druid of the Sacred Beaker");
    let deer_sub = reg.interner_mut().intern("Deer");
    let bird_sub = reg.interner_mut().intern("Bird");
    let ape_sub = reg.interner_mut().intern("Ape");
    let druid_sub = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(deer_sub);
    subtypes.0.insert(bird_sub);
    subtypes.0.insert(ape_sub);
    subtypes.0.insert(druid_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G} for each Crossbreed Labs watermark among permanents you control.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: add_mana,
            }),
    )
}

fn add_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each Crossbreed Labs watermark" — watermark detection not in API; adding 1 green as placeholder
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)] }]
}
