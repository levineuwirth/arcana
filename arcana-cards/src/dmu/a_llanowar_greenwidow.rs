//! A-Llanowar Greenwidow — `{2}{G}` 4/3 Spider with Reach and Trample.
//! Domain — `{5}{G}: Return Llanowar Greenwidow from your graveyard to the
//! battlefield tapped with a finality counter on it. This ability costs {1} less to
//! activate for each basic land type among lands you control.`
//!
//! Reach and Trample are base keywords. The Domain ability is a graveyard-activated
//! ability: it returns the card (by name) from graveyard to the battlefield via
//! Reanimate. The Domain cost reduction (no scalable-cost activation field), the
//! "tapped" rider, and the finality counter are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("A-Llanowar Greenwidow");
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}: Return Llanowar Greenwidow from your graveyard to the battlefield tapped with a finality counter on it."
                    .into(),
                cost: ActivationCost {
                    // GAP: Domain cost reduction ({1} less per basic land type) is
                    // not expressible as a scalable activation cost.
                    mana_cost: ManaCost::parse("{5}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self,
            }),
    )
}

fn return_self(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "tapped" rider and "with a finality counter on it" are not
    // expressible on the non-targeted graveyard return.
    let nm = reg.interner().lookup("A-Llanowar Greenwidow");
    vec![Effect::Reanimate {
        player: ctx.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        from_zone: Zone::Graveyard(ctx.controller),
    }]
}
