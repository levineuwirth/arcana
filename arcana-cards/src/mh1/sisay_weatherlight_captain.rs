//! Sisay, Weatherlight Captain — `{2}{W}` 2/2 Legendary Creature — Human
//! Soldier. White.
//! "Sisay gets +1/+1 for each color among other legendary permanents you
//! control." — a dynamic */* static continuous ability (no trigger, no cost),
//! not expressible here (GAP).
//! "{W}{U}{B}{R}{G}: Search your library for a legendary permanent card with
//! mana value less than Sisay's power, put that card onto the battlefield, then
//! shuffle." — TutorToBattlefield over a legendary filter with a dynamic
//! max-cmc computed from Sisay's current power.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sisay, Weatherlight Captain");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static — "Sisay gets +1/+1 for each color among other legendary
    // permanents you control" is a dynamic continuous ability, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}{U}{B}{R}{G}: Search your library for a legendary \
                   permanent card with mana value less than Sisay's power, put \
                   that card onto the battlefield, then shuffle."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_legendary,
        }),
    )
}

fn tutor_legendary(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // mana value less than Sisay's power → max_cmc = power - 1.
    let power = script::power_of(state, ctx.source).max(0) as u32;
    if power == 0 {
        return Vec::new();
    }
    let filter = ObjectFilter::permanent()
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))
        .with_max_cmc(power - 1);
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
