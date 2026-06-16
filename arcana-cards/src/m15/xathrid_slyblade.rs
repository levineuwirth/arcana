//! Xathrid Slyblade — `{2}{B}` 2/1 Creature — Human Assassin.
//! Hexproof.
//! `{3}{B}: Until end of turn, this creature loses hexproof and gains
//!   first strike and deathtouch.`
//!
//! Decomposition:
//! * keyword line → Hexproof.
//! * activated ability: mana cost {3}{B}; effect grants First Strike
//!   and Deathtouch to the source until end of turn. The "loses
//!   hexproof" clause has no single-keyword-removal primitive
//!   (LoseAllAbilities would strip the activated ability too), so that
//!   sub-clause is GAP'd while the two keyword grants are emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xathrid Slyblade");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{B}: Until end of turn, this creature loses hexproof and gains first strike and deathtouch.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_fs_dt,
        }),
    )
}

fn gain_fs_dt(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses hexproof" — no primitive removes a single named
    // keyword; the two grants below are emitted faithfully.
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
    ]
}
