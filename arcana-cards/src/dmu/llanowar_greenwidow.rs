//! Llanowar Greenwidow — `{2}{G}` 4/3 Spider with Reach and Trample.
//! "Domain — {7}{G}: Return this card from your graveyard to the
//!  battlefield tapped. It gains 'If this permanent would leave the
//!  battlefield, exile it instead of putting it anywhere else.' This
//!  ability costs {1} less to activate for each basic land type among
//!  lands you control."
//!
//! The graveyard recursion is expressible as a graveyard-activated ability
//! returning this card to the battlefield. The Domain "{1} less to activate
//! for each basic land type among lands you control" reduction is wired via
//! `ActivationCost::cost_reduction` (`script::domain`). The "tapped" rider
//! and the granted leave-the-battlefield replacement effect remain GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Llanowar Greenwidow");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Domain — "costs {1} less for each basic land type among lands
            //   you control" is wired via `cost_reduction`.
            // GAP (rider): "tapped" + grants the leave-battlefield-exile
            //   replacement effect — neither return-tapped nor a granted
            //   replacement is expressible here.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{7}{G}: Return this card from your graveyard to the battlefield tapped.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{7}{G}").expect("valid cost"),
                    cost_reduction: Some(domain_reduction),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_to_battlefield,
            }),
    )
}

/// Domain (CR 702.27) — "{1} less to activate for each basic land type
/// among lands you control." `controller` is the activator.
fn domain_reduction(
    state: &GameState,
    _source: ObjectId,
    controller: PlayerId,
    reg: &CardRegistry,
) -> u32 {
    script::domain(state, controller, reg)
}

fn return_self_to_battlefield(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
