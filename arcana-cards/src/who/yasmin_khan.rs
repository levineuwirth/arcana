//! Yasmin Khan — `{3}{R}` 3/3 Legendary Creature — Human Detective.
//!
//! * "{T}: Exile the top card of your library. Until your next end step, you may
//!   play it." — a tap-activated impulse: `Effect::ImpulseExile { count: 1 }`.
//!   (Fidelity: ImpulseExile's window is "until end of turn"; the printed window
//!   is "until your next end step" — the closest available primitive.)
//! * "Doctor's companion" — GAP: a commander-construction rule (you may have two
//!   commanders if the other is the Doctor); no gameplay primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yasmin Khan");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Exile the top card of your library. Until your next end step, you may play it."
                .into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: impulse_top,
        }),
    )
    // GAP: "Doctor's companion" — commander-deck-construction rule, no primitive.
}

fn impulse_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: ctx.controller,
        count: 1,
    }]
}
