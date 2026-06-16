//! Greater Morphling — `{6}{U}{U}` 5/5 blue Shapeshifter.
//! A stack of `{2}` activated abilities. The "gains your choice of …",
//! "becomes the colors of your choice", "becomes the creature type of
//! your choice", expansion-symbol, and art abilities all require a
//! player-chosen sub-mode the activation API can't express, so they're
//! GAP'd. The "+2/-2 or -2/+2" choice is also unmodeled. Only the
//! deterministic "Untap this creature" ability is expressible.

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
    let name = reg.interner_mut().intern("Greater Morphling");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "{2}: gains your choice of banding/bushido/double strike/…" —
    //      needs a player-chosen keyword sub-mode (no such activation form).
    // GAP: "{2}: becomes the colors of your choice" — chosen-color sub-mode.
    // GAP: "{2}: becomes the creature type of your choice" — chosen-type sub-mode.
    // GAP: "{2}: expansion symbol becomes the symbol of your choice" — cosmetic, unmodeled.
    // GAP: "{2}: art becomes by the artist of your choice" — cosmetic, unmodeled.
    // GAP: "{2}: gets +2/-2 or -2/+2" — player chooses which split; no choice form.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Untap this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
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

fn untap_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
