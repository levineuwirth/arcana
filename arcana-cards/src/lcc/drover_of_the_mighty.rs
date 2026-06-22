//! Drover of the Mighty — `{1}{G}` 1/1 Human Druid.
//! "This creature gets +2/+2 as long as you control a Dinosaur."
//! "{T}: Add one mana of any color."
//!
//! No keywords. The +2/+2-while-you-control-a-Dinosaur line is a
//! conditional static continuous ability (no trigger word, no cost) and
//! is GAP'd. The mana ability is wired ({T}, is_mana_ability) with a
//! fidelity GAP: "any color" has no player-chosen-color AddMana
//! primitive, so a fixed color (green) is produced.

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
    let name = reg.interner_mut().intern("Drover of the Mighty");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static — "This creature gets +2/+2 as long as you control a
    // Dinosaur" is a conditional continuous ability, not expressible as a
    // triggered/activated ability with the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_color,
            }),
    )
}

fn add_any_color(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (fidelity): "one mana of any color" — Effect::AddMana requires a
    // concrete ManaColor; the player's color choice is not modeled (green).
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
