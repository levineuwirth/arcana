//! Amphin Mutineer — `{3}{U}` 3/3 Salamander Pirate.
//! When this creature enters, exile up to one target non-Salamander creature.
//! That creature's controller creates a 4/3 blue Salamander Warrior token.
//! Encore {4}{U}{U} — GAP (keyword not in surface; the graveyard-activated
//! per-opponent token-copy ability is not expressible).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amphin Mutineer");
    let salamander = reg.interner_mut().intern("Salamander");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(salamander);
    subtypes.0.insert(pirate);

    // Pre-intern token subtypes so the resolver can rebuild them by lookup.
    let _warrior = reg.interner_mut().intern("Warrior");
    // Filter for "non-Salamander creature".
    let non_salamander = ObjectFilter::creature().without_subtype_sym(salamander);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword Encore is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_and_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(non_salamander),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_exile_and_token(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    // The token is created by the exiled creature's controller.
    let owner = script::target_controller(state, id, trig.controller);
    let salamander = reg.interner().lookup("Salamander").unwrap_or_default();
    let mut sub = SubtypeSet::default();
    sub.0.insert(salamander);
    if let Some(warrior) = reg.interner().lookup("Warrior") {
        sub.0.insert(warrior);
    }
    vec![
        Effect::ExilePermanent { target: id },
        Effect::CreateToken {
            controller: owner,
            token: TokenDefinition {
                name: salamander,
                colors: ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes: sub,
                power: Some(PtValue::Fixed(4)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
