//! Tomb Tyrant — `{3}{B}` 3/3 Zombie Noble.
//! "Other Zombies you control get +1/+1." (static, see GAP) and
//! "{2}{B}, {T}, Sacrifice a creature: Return a Zombie creature card
//! at random from your graveyard to the battlefield. Activate only
//! during your turn and only if there are at least three Zombie
//! creature cards in your graveyard."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tomb Tyrant");
    let zombie = reg.interner_mut().intern("Zombie");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(noble);

    // GAP: static "Other Zombies you control get +1/+1" is a continuous
    // anthem — no triggered/activated form to express here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}, Sacrifice a creature: Return a Zombie creature card at random from your graveyard to the battlefield. Activate only during your turn and only if there are at least three Zombie creature cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                // GAP: "only during your turn and only if >=3 Zombie
                // creature cards in your graveyard" — no exposed
                // activation_condition helper for those gates.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_zombie,
            }),
    )
}

fn reanimate_zombie(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let filter = ObjectFilter::creature().with_subtype_sym(zombie);
    vec![Effect::Reanimate {
        player: ctx.controller,
        filter,
        from_zone: Zone::Graveyard(ctx.controller),
    }]
}
