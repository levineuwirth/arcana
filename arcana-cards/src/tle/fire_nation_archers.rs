//! Fire Nation Archers — `{3}{R}` 3/4 Human Archer (red).
//! Reach.
//! "{5}: This creature deals 2 damage to each opponent. Create a 2/2 red
//! Soldier creature token."
//!
//! The activated ability deals 2 damage to each opponent (one DealDamage per
//! opponent) and then mints a 2/2 red Soldier token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire Nation Archers");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    // Pre-intern the token subtype so the resolver can look it up.
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}: This creature deals 2 damage to each opponent. Create a 2/2 red Soldier creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: burn_and_token,
            }),
    )
}

fn burn_and_token(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(opp),
            amount: 2,
        });
    }
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    effects.push(Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    });
    effects
}
