//! Bottle of Suleiman — `{4}` artifact (Arabian Nights).
//! "{1}, Sacrifice this artifact: Flip a coin. If you win the flip,
//! create a 5/5 colorless Djinn artifact creature token with flying.
//! If you lose the flip, this artifact deals 5 damage to you."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bottle of Suleiman");
    let _djinn = reg.interner_mut().intern("Djinn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, Sacrifice this artifact: Flip a coin. If you win the flip, create a 5/5 colorless Djinn artifact creature token with flying. If you lose the flip, this artifact deals 5 damage to you.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: flip_for_djinn,
            },
        ),
    )
}

fn flip_for_djinn(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let djinn = reg.interner().lookup("Djinn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: djinn,
                colors: ColorSet::new(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(5)),
                toughness: Some(PtValue::Fixed(5)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }),
        lose: Some(Box::new(Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 5,
            source: ctx.source,
        })),
    }]
}
