//! Last Stand — `{W}{U}{B}{R}{G}` sorcery. "Target opponent loses 2
//! life for each Swamp you control. Last Stand deals damage to target
//! creature equal to the number of Mountains you control. Create a 1/1
//! green Saproling creature token for each Forest you control. You gain
//! 2 life for each Plains you control. Draw a card for each Island you
//! control, then discard that many cards."
//!
//! Two targets: a target opponent (the life-loss) and a target creature
//! (the damage). Each magnitude scales off the count of a basic-land
//! subtype you control, computed at resolution with
//! `script::count_matching` over a subtype filter.

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Last Stand");
    let _saproling = reg.interner_mut().intern("Saproling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target opponent loses 2 life for each Swamp you control. \
                   Last Stand deals damage to target creature equal to the \
                   number of Mountains you control. Create a 1/1 green \
                   Saproling creature token for each Forest you control. You \
                   gain 2 life for each Plains you control. Draw a card for \
                   each Island you control, then discard that many cards."
                .into(),
            target_requirements: vec![
                TargetRequirement::target_player(),
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();

    // Target opponent loses 2 life for each Swamp you control.
    let swamps = script::count_matching(
        state,
        &script::subtype_filter(reg, "Swamp").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.first() {
        effects.push(Effect::LoseLife {
            player: *p,
            amount: swamps * 2,
        });
    }

    // Damage to target creature equal to the number of Mountains you control.
    let mountains = script::count_matching(
        state,
        &script::subtype_filter(reg, "Mountain").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.get(1) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: mountains,
        });
    }

    // Create a 1/1 green Saproling token for each Forest you control.
    let forests = script::count_matching(
        state,
        &script::subtype_filter(reg, "Forest").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if forests > 0 {
        let sap = reg
            .interner()
            .lookup("Saproling")
            .expect("Saproling interned during register()");
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(sap);
        let token = TokenDefinition {
            name: sap,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        };
        for _ in 0..forests {
            effects.push(Effect::CreateToken {
                controller: entry.controller,
                token: token.clone(),
            });
        }
    }

    // You gain 2 life for each Plains you control.
    let plains = script::count_matching(
        state,
        &script::subtype_filter(reg, "Plains").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    effects.push(Effect::GainLife {
        player: entry.controller,
        amount: plains * 2,
    });

    // Draw a card for each Island you control, then discard that many cards.
    let islands = script::count_matching(
        state,
        &script::subtype_filter(reg, "Island").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    effects.push(Effect::DrawCards {
        player: entry.controller,
        count: islands,
    });
    effects.push(Effect::Discard {
        player: entry.controller,
        count: islands,
        choice: DiscardChoice::ControllerChooses,
    });

    effects
}
