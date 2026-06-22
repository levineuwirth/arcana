//! Nettling Nuisance — `{2}{B}` 3/1 Faerie Rogue.
//! Flying.
//! "Whenever one or more Faeries you control deal combat damage to a player,
//! that player creates a 4/2 red Pirate creature token with 'This token can't
//! block.' The token is goaded for the rest of the game."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nettling Nuisance");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let _pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let faerie_filter = script::subtype_filter(reg, "Faerie")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever one or more Faeries you control deal combat damage to a
            // player, that player creates a 4/2 red Pirate token …"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: faerie_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: damaged_player_makes_pirate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damaged_player_makes_pirate(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(them) = trig.damaged_player() else {
        return Vec::new();
    };
    let pirate = reg.interner().lookup("Pirate").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pirate);
    // The 4/2 red Pirate token is minted under the damaged player's control.
    // GAP: the token's printed "This token can't block." static and its
    // "goaded for the rest of the game" rider are not expressible at creation
    // time (the new token's id isn't available to chain a Goad effect, and a
    // token-borne static can't be authored here).
    vec![Effect::CreateToken {
        controller: them,
        token: TokenDefinition {
            name: pirate,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
