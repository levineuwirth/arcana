//! Ifh-Bíff Efreet — `{2}{G}{G}` 3/3 Efreet with Flying.
//!
//! Oracle:
//! * Flying
//! * "{G}: This creature deals 1 damage to each creature with flying and each
//!   player. Any player may activate this ability."
//!
//! GAP: "Any player may activate this ability" — the activation is wired as a
//! normal controller-activated ability; there is no way to grant activation
//! rights to all players. The damage payload is fully expressed (1 to every
//! flying creature and every player).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ifh-Bíff Efreet");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{G}: This creature deals 1 damage to each creature with flying and each player. Any player may activate this ability.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: damage_flyers_and_players,
        }),
    )
}

fn damage_flyers_and_players(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    // 1 damage to each creature with flying.
    let flyers = script::ids_matching(
        state,
        &ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
        ctx.controller,
    );
    for id in flyers {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 1,
        });
    }
    // 1 damage to each player.
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}
